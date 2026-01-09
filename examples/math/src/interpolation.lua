-- Interpolation and mapping utilities

local M = {}

-- Need basic module for clamp
-- Assuming file structure allows relative require or passed via dependencies if needed.
-- For simplicity in Lua, we'll duplicate local clamp or assume user imports correctly.
-- But cleanest is to require basic here if the environment supports it.
-- Since this is an odrill project, let's assume standard require works or we define local helper.

local function clamp(value, min, max)
    if value < min then return min
    elseif value > max then return max
    end
    return value
end

--- Linear interpolation between two values
--- @param a number Start value
--- @param b number End value
--- @param t number Interpolation factor (0-1)
--- @return number Interpolated value
function M.lerp(a, b, t)
    return a + (b - a) * t
end

--- Remap a value from one range to another
--- @param value number The value to remap
--- @param in_min number Input range minimum
--- @param in_max number Input range maximum
--- @param out_min number Output range minimum
--- @param out_max number Output range maximum
--- @return number Remapped value
function M.map_range(value, in_min, in_max, out_min, out_max)
    return (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
end

--- Hermite interpolation between two values (smooth easing)
--- @param edge0 number Lower edge
--- @param edge1 number Upper edge
--- @param x number Input value
--- @return number Interpolated value
function M.smoothstep(edge0, edge1, x)
    local t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0)
    return t * t * (3.0 - 2.0 * t)
end

return M
