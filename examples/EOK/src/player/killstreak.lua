-- ============================================================================
-- EOK Player Manager - Killstreak System
-- Tracks kills and combo multiplier
-- ============================================================================

-- Killstreak counter
local function add_killstreak(player)
    player._killstreak = (player._killstreak or 0) + 1
    return player._killstreak
end

-- Reset killstreak
local function reset_killstreak(player)
    player._killstreak = nil
end

-- Get current killstreak
local function get_killstreak(player)
    return player._killstreak or 0
end
