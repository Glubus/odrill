-- ============================================================================
-- EOK Enemy Multipliers
-- XP multipliers for special enemy types
-- ============================================================================

use specials::is_special

-- EOK's custom XP multipliers per enemy type
local MULTIPLIERS = {
    -- Snipers
    heavy_swat_sniper = 1.2,
    sniper = 1.2,
    
    -- Shields  
    shield = 1.2,
    marshal_shield = 1.3,
    marshal_shield_break = 1.2,
    
    -- Support
    medic = 1.3,
    tank_medic = 1.5,
    taser = 1.3,
    
    -- Cloakers
    spooc = 1.4,
    shadow_spooc = 1.5,
    
    -- Phalanx
    phalanx_minion = 1.3,
    phalanx_vip = 1.8,
    
    -- Bosses
    drug_lord_boss = 2.0,
    drug_lord_boss_stealth = 2.0,
    ranchmanager = 2.0,
    triad_boss = 2.0,
    triad_boss_no_armor = 1.8,
    deep_boss = 2.0,
    snowman_boss = 2.0,
    
    -- Bulldozers
    piggydozer = 1.8,
    tank = 1.5,
    tank_mini = 1.3,
    tank_hw = 1.6
}

-- Default multiplier for unlisted specials
local DEFAULT_SPECIAL = 1.2

-- Get XP multiplier for an enemy type
local function get_multiplier(enemy_type)
    if not is_special(enemy_type) then
        return 1.0
    end
    return MULTIPLIERS[enemy_type] or DEFAULT_SPECIAL
end

-- Check if enemy type gives bonus XP
local function gives_bonus(enemy_type)
    return is_special(enemy_type)
end

-- Export functions
return {
    get_multiplier = get_multiplier,
    gives_bonus = gives_bonus
}
