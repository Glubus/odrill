-- ============================================================================
-- EOK Utilities
-- Helper functions used throughout the mod
-- ============================================================================

use math::round

-- Make text fit its content
local function make_fine_text(text)
    local x, y, w, h = text:text_rect()
    text:set_size(w, h)
    text:set_position(round(text:x()), round(text:y()))
end

-- Get HUD panel safely
local function get_hud_panel()
    local hud = managers.hud:script(PlayerBase.PLAYER_INFO_HUD_FULLSCREEN_PD2)
    if hud and hud.panel then
        return hud, hud.panel
    end
    return nil, nil
end

-- Remove panel child safely
local function remove_panel_child(panel, child_name)
    local child = panel:child(child_name)
    if child and alive(child) then
        panel:remove(child)
        return true
    end
    return false
end

-- Check if special bonus is active
local function is_special_active()
    return managers.player:has_active_temporary_property("bonus_xp_special_killed_EOK")
end
