-- ============================================================================
-- EOK PlayerManager Hook
-- Tracks kills, calculates XP, and awards experience on kill
-- ============================================================================

use mods::enemies::get_multiplier
use mods::config::get_option

-- ============================================================================
-- INITIALIZATION
-- ============================================================================

Hooks:PostHook(PlayerManager, "check_skills", "EOK_init_player_vars", function(self)
    self._total_xp_gained_on_kill = nil
    self._xp_combo = 0
    self._xp_combo_t = 0
    self._killstreak = nil
end)

-- ============================================================================
-- XP MANAGEMENT
-- ============================================================================

-- Main XP calculation function (exposed for HUD preview)
function PlayerManager:_exp_gain_manage(ui)
    local difficulty = Global.game_settings and Global.game_settings.difficulty or "normal"
    local base_exp = 100
    
    -- Difficulty multipliers
    local diff_mults = {
        easy = 0.7,
        normal = 0.9,
        hard = 1.0,
        overkill = 1.2,
        overkill_145 = 2.0,
        easy_wish = 3.0,
        overkill_290 = 4.0,
        sm_wish = 5.0
    }
    
    base_exp = base_exp * (diff_mults[difficulty] or 1.0)
    
    -- Soft cap system
    local soft_caps = {
        easy = 4200,
        normal = 4200,
        hard = 4200,
        overkill = 6500,
        overkill_145 = 13500,
        easy_wish = 17500,
        overkill_290 = 25000,
        sm_wish = 31000
    }
    
    local current_cap = soft_caps[difficulty] or 4200
    
    -- Infamy scaling
    if managers.experience then
        local rank = managers.experience:current_rank() or 0
        current_cap = current_cap * (1 + (rank * 0.05))
    end
    
    -- Apply soft cap reduction if over cap
    if self._total_xp_gained_on_kill and self._total_xp_gained_on_kill > current_cap then
        if get_option("gain_exp_on_kills", true) then
            local reduction = 0.95 - ((self._total_xp_gained_on_kill - current_cap) / self._total_xp_gained_on_kill)
            reduction = math.max(reduction, 0.01)
            base_exp = base_exp * reduction
        end
    end
    
    -- Enforce minimum XP
    base_exp = math.max(base_exp, 5)
    
    -- Track total XP (unless UI preview)
    if not ui then
        self._total_xp_gained_on_kill = (self._total_xp_gained_on_kill or 0) + base_exp
    end
    
    return base_exp
end

-- ============================================================================
-- SPECIAL ENEMY DETECTION
-- ============================================================================

function PlayerManager:_delayed_clbk_remove_bonus()
    DelayedCalls:Add("bonus_exp_anim_EOK", self:get_temporary_property("bonus_xp_special_killed_EOK", 0), function()
        if managers.hud and managers.hud._force_remove_bonus_xp then
            managers.hud:_force_remove_bonus_xp()
        end
        if managers.hud and managers.hud._force_remove_bonus_modern then
            managers.hud:_force_remove_bonus_modern()
        end
    end)
end

function PlayerManager:_chk_special_cop_exp_gain(unit)
    if not (unit and alive(unit) and unit.base and type(unit.base) == "function") then
        return nil
    end
    
    local base = unit:base()
    if not (base and base._tweak_table) then
        return nil
    end
    
    local enemy_type = base._tweak_table
    local multiplier = get_multiplier(enemy_type)
    
    if multiplier and multiplier > 1.0 then
        self:activate_temporary_property("bonus_xp_special_killed_EOK", 8, 8)
        self:_delayed_clbk_remove_bonus()
        return multiplier
    end
    
    return nil
end

-- ============================================================================
-- KILLSTREAK TRACKING
-- ============================================================================

function PlayerManager:_killstreak_add()
    self._killstreak = (self._killstreak or 0) + 1
    return self._killstreak
end

function PlayerManager:_killstreak_reset()
    self._killstreak = nil
end

-- ============================================================================
-- MAIN KILL HANDLER
-- ============================================================================

function PlayerManager:_do_exp_on_kill(killed_unit, variant, headshot)
    if not get_option("gain_exp_on_kills", true) then
        return
    end
    
    local exp = self:_exp_gain_manage()
    local special_multiplier = self:_chk_special_cop_exp_gain(killed_unit)
    
    -- Special Enemy Bonus
    if special_multiplier then
        exp = exp * special_multiplier
    elseif self:has_active_temporary_property("bonus_xp_special_killed_EOK") then
        exp = exp * 1.2
    end
    
    -- Headshot Bonus
    if headshot then
        exp = exp * 1.2
    end
    
    -- Combo System
    local current_time = Application:time()
    if self._xp_combo_t and (current_time - self._xp_combo_t) > 5.0 then
        self._xp_combo = 0
    end
    self._xp_combo_t = current_time
    
    self._xp_combo = math.min((self._xp_combo or 0) + 0.01, 1.0)
    exp = exp * (1 + self._xp_combo)
    
    -- Track killstreak and display HUD
    local kills = self:_killstreak_add()
    if managers.hud and managers.hud._kill_exp then
        managers.hud:_kill_exp(kills, special_multiplier ~= nil)
    end
    
    -- Track kill XP for end screen
    if Global.experience_manager then
        local current = Global.experience_manager.kills_xp or 0
        Global.experience_manager.kills_xp = current + math.round(exp)
    end
    
    -- Award XP
    managers.experience:mission_xp_award(exp)
end

-- ============================================================================
-- HOOK INTO GAME
-- ============================================================================

Hooks:PreHook(PlayerManager, "on_killshot", "EOK_on_killshot", function(self, killed_unit, variant, headshot, weapon_id)
    if not self:player_unit() then
        return
    end
    if not killed_unit then
        return
    end
    self:_do_exp_on_kill(killed_unit, variant, headshot)
end)
