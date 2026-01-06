-- ============================================================================
-- Odrill Math Library
-- Math utilities missing from Payday 2 Lua
-- Author: Odrill Official Team
-- ============================================================================

--- Round a number to specified decimal places
--- @param num number The number to round
--- @param decimals number Optional decimal places (default: 0)
--- @return number The rounded number
local function round(num, decimals)
    local mult = 10 ^ (decimals or 0)
    if num >= 0 then
        return math.floor(num * mult + 0.5) / mult
    else
        return math.ceil(num * mult - 0.5) / mult
    end
end

--- Clamp a value between min and max
--- @param value number The value to clamp
--- @param min number Minimum bound
--- @param max number Maximum bound
--- @return number The clamped value
local function clamp(value, min, max)
    if value < min then
        return min
    elseif value > max then
        return max
    end
    return value
end

--- Linear interpolation between two values
--- @param a number Start value
--- @param b number End value
--- @param t number Interpolation factor (0-1)
--- @return number Interpolated value
local function lerp(a, b, t)
    return a + (b - a) * t
end

--- Remap a value from one range to another
--- @param value number The value to remap
--- @param in_min number Input range minimum
--- @param in_max number Input range maximum
--- @param out_min number Output range minimum
--- @param out_max number Output range maximum
--- @return number Remapped value
local function map_range(value, in_min, in_max, out_min, out_max)
    return (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
end

--- Get the sign of a number
--- @param n number The number
--- @return number -1, 0, or 1
local function sign(n)
    if n > 0 then
        return 1
    elseif n < 0 then
        return -1
    end
    return 0
end
