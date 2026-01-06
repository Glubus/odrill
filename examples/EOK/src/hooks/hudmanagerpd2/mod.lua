-- ============================================================================
-- EOK HUDManager Hook
-- Displays XP gains on screen using classic or modern HUD style
-- ============================================================================

use mods::hud::classic::mod::display_classic
use mods::hud::modern::mod::display_modern
use mods::config::get_option

-- ============================================================================
-- MAIN HUD DISPLAY FUNCTION
-- ============================================================================

function HUDManager:_kill_exp(killstreak, was_special)
    -- Check which style to use
    local style = get_option("anim_choice_style", 1)
    
    if style == 2 then
        display_modern(self, killstreak, was_special)
    else
        display_classic(self, killstreak, was_special)
    end
end

-- ============================================================================
-- CLEANUP FUNCTIONS
-- ============================================================================

-- Force remove bonus XP text (classic style)
function HUDManager:_force_remove_bonus_xp()
    if not self._exp_bonus then return end
    
    local hud = managers.hud:script(PlayerBase.PLAYER_INFO_HUD_FULLSCREEN_PD2)
    if not hud or not hud.panel then return end
    
    local panel = hud.panel
    local child = panel:child("exp_bonus_text____")
    if not child or not alive(child) then return end
    if self.played_anim_b then
        self._bonus_xp_earned = nil
        return
    end
    
    local bonus_xp = self._exp_bonus
    local fade_dir = get_option("exp_fade_out_style", 1) - 1
    
    bonus_xp:animate(function(o)
        local t = 1
        while t > 0 and alive(bonus_xp) do
            local dt = coroutine.yield()
            t = math.clamp(t - dt, 0, 1)
            o:move(fade_dir * dt * 20, (1 - math.abs(fade_dir)) * -dt * 20)
            bonus_xp:set_alpha(t)
        end
        if alive(bonus_xp) then bonus_xp:hide() end
    end)
    
    self._bonus_xp_earned = nil
    self:BONUS_PLAYED_ANIM(true)
end

-- Force remove bonus XP text (modern style)
function HUDManager:_force_remove_bonus_modern()
    if self._bonus_exp_text then
        local hud = managers.hud:script(PlayerBase.PLAYER_INFO_HUD_FULLSCREEN_PD2)
        if hud and hud.panel then
            local panel = hud.panel
            local child = panel:child("bonus_exp_text")
            if child and alive(child) then
                panel:remove(child)
            end
        end
    end
end

-- Bonus animation state tracker
function HUDManager:BONUS_PLAYED_ANIM(chk)
    self.played_anim_b = chk
end
