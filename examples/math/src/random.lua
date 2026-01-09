-- Randomness utilities

local M = {}

--- Return true with a given percentage chance
--- @param probability number Chance between 0.0 and 1.0 (e.g., 0.5 for 50%)
--- @return boolean True if the chance check passed
function M.chance(probability)
    return math.random() < probability
end

--- Return a random float between min and max
--- @param min number Minimum value
--- @param max number Maximum value
--- @return number Random float
function M.random_float(min, max)
    return min + math.random() * (max - min)
end

return M
