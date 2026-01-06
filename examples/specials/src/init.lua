-- ============================================================================
-- Odrill Specials Library
-- Payday 2 special enemy types
-- Author: Odrill Official Team
-- ============================================================================

-- Complete list of special enemy types in Payday 2
local ENEMIES = {
    -- Snipers
    "heavy_swat_sniper",
    "sniper",
    
    -- Shields
    "shield",
    "marshal_shield",
    "marshal_shield_break",
    
    -- Support
    "medic",
    "tank_medic",
    "taser",
    
    -- Cloakers
    "spooc",
    "shadow_spooc",
    
    -- Phalanx
    "phalanx_minion",
    "phalanx_vip",
    
    -- Bosses
    "drug_lord_boss",
    "drug_lord_boss_stealth",
    "ranchmanager",
    "triad_boss",
    "triad_boss_no_armor",
    "deep_boss",
    "snowman_boss",
    
    -- Bulldozers
    "piggydozer",
    "tank",
    "tank_mini",
    "tank_hw",
}

-- Create lookup table for O(1) checks
local ENEMIES_SET = {}
for _, enemy in ipairs(ENEMIES) do
    ENEMIES_SET[enemy] = true
end

--- Check if an enemy type is a special enemy
--- @param enemy_type string The enemy tweak table name
--- @return boolean True if special enemy
local function is_special(enemy_type)
    return ENEMIES_SET[enemy_type] == true
end
