-- ============================================================================
-- Modern HUD - Complete Module
-- Modernized XP display with counter animations
-- ============================================================================

use mods::hud::classic::animations::*
use mods::hud::classic::texts::*
use mods::hud::colors::{pack_color, is_rainbow, animate_rainbow}
use mods::utils::make_fine_text
use mods::utils::get_hud_panel
use mods::utils::is_special_active
use math::round
use mods::config::get_option

-- Cleanup state
local function cleanup(self)
    self._total_exp = nil
    self._total_exp_text = nil
    self._exp_gained_text = nil
    self._bonus_exp_text = nil
    self._bonus_exp_gained_total = nil
    managers.player:_killstreak_reset()
end

-- Force remove bonus text
function HUDManager:_force_remove_bonus_modern()
    if self._bonus_exp_text then
        local hud, panel = get_hud_panel()
        if panel then
            remove_text(panel, "bonus_exp_text")
        end
    end
end

-- Main modern kill XP display
local function display_modern(self, killstreak, was_special)
    if get_option("disable_hud_effects", false) then
        return
    end

    local hud, panel = get_hud_panel()
    if not panel then return end

    was_special = was_special or is_special_active()

    -- Cleanup existing
    remove_text(panel, "total_exp_text")
    remove_text(panel, "exp_gained_text")
    if was_special then
        remove_text(panel, "bonus_exp_text")
    end

    -- Calculate XP
    local total_exp = self._total_exp or 0
    local exp_gained = round(managers.player:_exp_gain_manage(true), 1)
    self._total_exp = total_exp + exp_gained

    local x_off = get_option("exp_adjust_x", 0)
    local y_off = get_option("exp_adjust_y", 0)
    local center_w = panel:w() / 2
    local center_h = panel:h() / 2

    -- Total XP text
    self._total_exp_text = panel:text({
        name = "total_exp_text",
        y = center_h - 50, h = 50, w = panel:w(),
        font_size = get_option("exp_font_size", 15) * 1.2,
        vertical = "center", align = "center",
        font = "fonts/font_medium_shadow_mf",
        render_template = "OverlayVertexColorTextured",
        text = tostring(total_exp),
        color = pack_color(get_option("exp_color", 14))
    })

    -- Gained XP text
    self._exp_gained_text = panel:text({
        name = "exp_gained_text",
        y = center_h - 30, h = 40, w = panel:w(),
        font_size = get_option("exp_font_size", 15),
        vertical = "center", align = "center",
        font = "fonts/font_medium_shadow_mf",
        render_template = "OverlayVertexColorTextured",
        text = "+" .. tostring(exp_gained),
        color = pack_color(get_option("exp_color", 2))
    })

    -- Bonus text for special kills
    if was_special then
        local bonus = (exp_gained * 1.2) - exp_gained
        self._bonus_exp_gained_total = (self._bonus_exp_gained_total or 0) + bonus
        self._bonus_exp_text = panel:text({
            name = "bonus_exp_text",
            y = center_h, h = 40, w = panel:w(),
            font_size = get_option("exp_font_size", 15),
            vertical = "center", align = "center",
            font = "fonts/font_medium_shadow_mf",
            render_template = "OverlayVertexColorTextured",
            text = "+" .. tostring(round(bonus, 1)),
            color = pack_color(get_option("specialkilled_color", 2))
        })
        make_fine_text(self._bonus_exp_text)
        self._bonus_exp_text:set_center(center_w + 25 + x_off, center_h + 40 + y_off)
    end

    make_fine_text(self._total_exp_text)
    make_fine_text(self._exp_gained_text)
    self._total_exp_text:set_center(center_w + 20 + x_off, center_h + y_off)
    self._exp_gained_text:set_center(center_w + 25 + x_off, center_h + 20 + y_off)

    -- Rainbow animations
    if is_rainbow(get_option("exp_color", 14)) then
        self._total_exp_text:animate(animate_rainbow)
    end
    if was_special and self._bonus_exp_text and is_rainbow(get_option("specialkilled_color", 2)) then
        self._bonus_exp_text:animate(animate_rainbow)
    end

    -- Counter animation
    local end_exp = self._total_exp + (self._bonus_exp_gained_total or 0)
    local total_text = self._total_exp_text
    total_text:animate(function()
        local t = 0.5
        while t > 0 and alive(total_text) do
            local dt = coroutine.yield()
            t = math.clamp(t - dt, 0, 1)
            local current = math.lerp(total_exp, end_exp, 1 - t)
            total_text:set_text(tostring(round(current, 1)))
            make_fine_text(total_text)
        end
        total_text:set_text(tostring(end_exp) .. " »XP")
        make_fine_text(total_text)
    end)

    -- Fade gained text
    local gained = self._exp_gained_text
    gained:animate(function(o)
        if is_rainbow(get_option("exp_color", 14)) then
            gained:animate(animate_rainbow)
        end
        local t = 0.8
        while t > 0 and alive(o) do
            local dt = coroutine.yield()
            t = math.clamp(t - dt, 0, 1)
            o:set_alpha(t)
            o:set_y(o:y() - dt * 60)
        end
        panel:remove(o)
    end)

    -- Fade bonus text
    if was_special and self._bonus_exp_text then
        local bonus = self._bonus_exp_text
        bonus:animate(function(o)
            local t = 0.8
            while t > 0 and alive(o) do
                local dt = coroutine.yield()
                t = math.clamp(t - dt, 0, 1)
                o:set_alpha(t)
                o:set_y(o:y() - dt * 60)
            end
            panel:remove(o)
        end)
    end

    -- Delayed fade for total
    DelayedCalls:Add("fade_out_total_text", get_option("kln_duration", 2) + 1, function()
        if alive(total_text) then
            total_text:animate(function(o)
                local t = 0.5
                while t > 0 and alive(o) do
                    local dt = coroutine.yield()
                    t = math.clamp(t - dt, 0, 0.5)
                    o:set_alpha(t * 2)
                    o:set_y(o:y() - dt * 50)
                end
                if alive(o) then panel:remove(o) end
            end)
        end
        cleanup(self)
    end)
end

-- Export function
return {
    display_modern = display_modern
}
