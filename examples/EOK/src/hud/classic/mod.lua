-- ============================================================================
-- Classic HUD - Main Module
-- Full classic XP display matching original mod
-- ============================================================================

use mods::hud::classic::animations::*
use mods::hud::classic::texts::*
use mods::hud::colors::*
use math::round
use mods::utils::make_fine_text
use mods::utils::get_hud_panel
use mods::utils::is_special_active
use math::round
use mods::config::get_option

-- Cleanup state
local function cleanup(self)
    self._exp_gained = nil
    self._bonus_xp_earned = nil
    self._previous_num = nil
    self._block_anim = nil
    managers.player:_killstreak_reset()
end

-- Force remove bonus XP text
function HUDManager:_force_remove_bonus_xp()
    if not self._exp_bonus then return end
    
    local hud, panel = get_hud_panel()
    if not panel then return end
    
    local child = panel:child("exp_bonus_text____")
    if not child or not alive(child) then return end
    if self.played_anim_b then
        self._bonus_xp_earned = nil
        return
    end
    
    local bonus_xp = self._exp_bonus
    local fade_dir = get_option("exp_fade_out_style", 1) - 1
    
    fade_out_move(bonus_xp, fade_dir, function()
        self._bonus_xp_earned = nil
        self:BONUS_PLAYED_ANIM(true)
    end)
end

function HUDManager:BONUS_PLAYED_ANIM(chk)
    self.played_anim_b = chk
end

-- Main classic kill XP display
local function display_classic(self, killstreak, was_special)
    if get_option("disable_hud_effects", false) then
        return
    end

    local hud, panel = get_hud_panel()
    if not panel then return end

    local base_exp = round(managers.player:_exp_gain_manage(true), 1)
    local shrt = get_shortcut(get_option("shortcut_choice_exp", 2))

    self._block_anim = self._block_anim or false
    self._previous_num = self._previous_num or 0
    self._exp_gained = self._exp_gained or 0
    was_special = was_special or is_special_active()

    if was_special then
        self.played_anim_b = false
    end

    -- Cleanup existing
    remove_text(panel, "exp_text____")
    remove_text(panel, "killstreak_text__")
    if was_special then
        remove_text(panel, "exp_bonus_text____")
    end

    local x_off = get_option("exp_adjust_x", 0)
    local y_off = get_option("exp_adjust_y", 0)
    local center_w = panel:w() / 2
    local center_h = panel:h() / 2

    -- Bonus text for special kills
    if was_special then
        local bonus_calc = (base_exp * 1.2) - base_exp
        self._bonus_xp_earned = (self._bonus_xp_earned or 0) + bonus_calc
        
        self._exp_bonus = create_text(panel, {
            name = "exp_bonus_text____",
            y = center_h - 50,
            font_size = get_option("exp_font_size", 15),
            text = "+" .. tostring(round(self._bonus_xp_earned, 1)) .. shrt,
            color = pack_color(get_option("specialkilled_color", 2), true)
        })
    end

    -- Main XP text
    self._exp_gained = self._exp_gained + base_exp
    self._exp = create_text(panel, {
        name = "exp_text____",
        y = center_h - 50,
        font_size = get_option("exp_font_size", 15),
        text = "+" .. tostring(self._exp_gained) .. shrt,
        color = pack_color(get_option("exp_color", 14))
    })

    -- Killstreak text
    local kln_name = get_killstreak_name(killstreak)
    self._killstreak_ann = create_text(panel, {
        name = "killstreak_text__",
        y = center_h,
        font_size = get_option("kln_font_size", 15),
        text = kln_name or "DOUBLE KILL",
        color = pack_color(get_option("exp_color", 14)),
        alpha = 0.9,
        visible = kln_name ~= nil and get_option("show_announcer", true)
    })

    -- Position elements
    local text = self._exp
    local kln = self._killstreak_ann
    local bonus_xp = self._exp_bonus

    text:set_center(center_w + 20 + x_off, center_h + 20 + y_off)

    if was_special and bonus_xp then
        bonus_xp:set_center(text:x() + 15, text:y() - 5)
    end

    local kln_x = center_w + text:x() + 20 + get_option("kln_adjust_x", 0) + 50
    kln:set_center(kln_x, text:y() + 28 + get_option("kln_adjust_y", 0))

    -- Rainbow animations
    if is_rainbow(get_option("exp_color", 14)) then
        self._exp:animate(animate_rainbow)
        self._killstreak_ann:animate(animate_rainbow)
    end
    if was_special and bonus_xp and is_rainbow(get_option("specialkilled_color", 2)) then
        bonus_xp:animate(animate_rainbow)
    end

    -- Track killstreak
    local function block_anim(set)
        if set and self._previous_num < killstreak then
            self._previous_num = killstreak
        end
        self._block_anim = set
    end

    -- Fade out animations
    local fade_dir = get_option("exp_fade_out_style", 1) - 1

    DelayedCalls:Add("text__anim", get_option("exp_text_duration", 5), function()
        if alive(text) then
            fade_out_move(text, fade_dir)
        end

        if alive(bonus_xp) and not self.played_anim_b then
            fade_out_move(bonus_xp, fade_dir, function()
                self:BONUS_PLAYED_ANIM(true)
            end)
        end

        if not is_special_active() then
            self._bonus_xp_earned = nil
        end
        self._exp_gained = nil
        managers.player:_killstreak_reset()
    end)

    DelayedCalls:Add("kln__anim", get_option("kln_duration", 2), function()
        if alive(kln) then
            slide_out(kln, function()
                block_anim(false)
                self._bonus_xp_earned = nil
                self._previous_num = nil
            end)
        end
    end)
end

-- Export function
return {
    display_classic = display_classic
}
