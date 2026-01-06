-- ============================================================================
-- Classic HUD - Text Factory
-- Creates and manages text elements
-- ============================================================================

use math::round
use mods::utils::make_fine_text

-- Killstreak names
local KILLSTREAK_NAMES = {"DOUBLE", "TRIPLE", "QUADRA", "PENTA", "HEXA"}
local SHORTCUT_OPTIONS = {" EXP", " »XP"}

-- Get killstreak name for count
local function get_killstreak_name(count)
    if count >= 2 and count <= 6 then
        return (KILLSTREAK_NAMES[count - 1] or "MULTI") .. " KILL"
    elseif count > 6 then
        return "BLOODTHIRSTY!"
    end
    return nil
end

-- Get XP shortcut text
local function get_shortcut(choice)
    return SHORTCUT_OPTIONS[choice or 2]
end

-- Create styled text element
local function create_text(panel, params)
    local text = panel:text({
        name = params.name,
        y = params.y or 0,
        h = params.h or 50,
        w = panel:w(),
        font_size = params.font_size or 15,
        vertical = "center",
        align = "center",
        font = "fonts/font_medium_shadow_mf",
        render_template = "OverlayVertexColorTextured",
        text = params.text or "",
        color = params.color or Color.white,
        alpha = params.alpha or 1,
        visible = params.visible ~= false
    })
    make_fine_text(text)
    return text
end

-- Remove text safely
local function remove_text(panel, name)
    local child = panel:child(name)
    if child and alive(child) then
        panel:remove(child)
        return true
    end
    return false
end
