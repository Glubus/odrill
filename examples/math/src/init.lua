-- ============================================================================
-- Odrill Math Library
-- Math utilities missing from Payday 2 Lua
-- Author: Odrill Official Team
-- ============================================================================

local basic = require("src/basic")
local random = require("src/random")
local interpolation = require("src/interpolation")

-- Aggregate all functions into the main export
local M = {}

for k, v in pairs(basic) do M[k] = v end
for k, v in pairs(random) do M[k] = v end
for k, v in pairs(interpolation) do M[k] = v end

return M
