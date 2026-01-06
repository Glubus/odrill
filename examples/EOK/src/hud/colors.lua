-- ============================================================================
-- EOK Colors System
-- Color palette and rainbow animation for XP display
-- ============================================================================

use colors::{PALETTE, rainbow}
use math::round

-- Color indices for config
local COLOR_INDEX = {
    "PASTEL_PINK", "PURPLE", "AQUA", "STRAWBERRY", "ORANGE",
    "RED", "NAVY", "PINK", "LILAC", "BLACK",
    "BLUE_VIOLET", "WHITE", "GREEN", "YELLOW", "PALE_YELLOW"
}

local RAINBOW_INDEX = 16
local RANDOM_INDEX = 17

-- Get RGB from color index
local function color_from_index(value, is_special)
    if value == RAINBOW_INDEX then
        return 1, 1, 1  -- Will be animated
    end
    
    if value == RANDOM_INDEX then
        local key = COLOR_INDEX[math.random(#COLOR_INDEX)]
        local c = PALETTE[key]
        return c[1] / 255, c[2] / 255, c[3] / 255
    end
    
    if not value then
        local default = is_special and "PURPLE" or "PALE_YELLOW"
        local c = PALETTE[default]
        return c[1] / 255, c[2] / 255, c[3] / 255
    end
    
    local name = COLOR_INDEX[value]
    if name and PALETTE[name] then
        local c = PALETTE[name]
        return c[1] / 255, c[2] / 255, c[3] / 255
    end
    return 1, 1, 1
end

-- Pack into Color object
local function pack_color(value, is_special)
    local r, g, b = color_from_index(value, is_special)
    return Color(r, g, b)
end

-- Check if rainbow mode
local function is_rainbow(value)
    return value == RAINBOW_INDEX
end

-- Animate element with rainbow colors
local function animate_rainbow(text_element)
    while alive(text_element) do
        local r, g, b = rainbow()
        text_element:set_color(Color(r, g, b))
        coroutine.yield()
    end
end
