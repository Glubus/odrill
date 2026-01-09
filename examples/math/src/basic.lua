-- Basic math utilities

local M = {}

--- Round a number to specified decimal places
--- @param num number The number to round
--- @param decimals number Optional decimal places (default: 0)
--- @return number The rounded number
function M.round(num, decimals)
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
function M.clamp(value, min, max)
    if value < min then
        return min
    elseif value > max then
        return max
    end
    return value
end

--- Check if a value is between two bounds (inclusive)
--- @param value number The value to check
--- @param min number Minimum bound
--- @param max number Maximum bound
--- @return boolean True if min <= value <= max
function M.between(value, min, max)
    return value >= min and value <= max
end

--- Get the sign of a number
--- @param n number The number
--- @return number -1, 0, or 1
function M.sign(n)
    if n > 0 then
        return 1
    elseif n < 0 then
        return -1
    end
    return 0
end

--- Get the minimum value from a table
--- @param t table The table/list of numbers
--- @return number|nil The minimum value found, or nil if empty
function M.table_min(t)
    if #t == 0 then return nil end
    local min_val = t[1]
    for i = 2, #t do
        if t[i] < min_val then
            min_val = t[i]
        end
    end
    return min_val
end

--- Get the maximum value from a table
--- @param t table The table/list of numbers
--- @return number|nil The maximum value found, or nil if empty
function M.table_max(t)
    if #t == 0 then return nil end
    local max_val = t[1]
    for i = 2, #t do
        if t[i] > max_val then
            max_val = t[i]
        end
    end
    return max_val
end

return M
