-- ============================================================================
-- EOK Player Manager - XP Calculation
-- XP calculation logic with difficulty scaling
-- ============================================================================

-- Configuration constants
local CONFIG = {
    BASE_XP = 100,
    MIN_XP = 5,
    DEFAULT_DIFFICULTY = "normal",
    
    -- Difficulty multipliers
    DIFFICULTY_MULTIPLIERS = {
        easy = 0.7,
        normal = 0.9,
        hard = 1.0,
        overkill = 1.2,
        overkill_145 = 2.0,
        easy_wish = 3.0,
        overkill_290 = 4.0,
        sm_wish = 5.0
    },
    
    -- Soft caps per difficulty
    SOFT_CAPS = {
        easy = 4200,
        normal = 4200,
        hard = 4200,
        overkill = 6500,
        overkill_145 = 13500,
        easy_wish = 17500,
        overkill_290 = 25000,
        sm_wish = 31000
    },
    DEFAULT_SOFT_CAP = 4200,
    
    -- Scaling
    INFAMY_CAP_BONUS = 0.05,
    SOFT_CAP_REDUCTION = 0.95,
    SOFT_CAP_MIN = 0.01,
    
    -- Combat bonuses
    HEADSHOT_BONUS = 1.2,
    COMBO_INCREMENT = 0.01,
    COMBO_MAX = 1.0,
    COMBO_DECAY = 5.0,
    
    -- Special enemy
    SPECIAL_BONUS_DURATION = 8
}

-- Get current difficulty
local function get_difficulty()
    return Global.game_settings and Global.game_settings.difficulty or CONFIG.DEFAULT_DIFFICULTY
end

-- Get difficulty multiplier
local function get_difficulty_multiplier(difficulty)
    return CONFIG.DIFFICULTY_MULTIPLIERS[difficulty] or 1.0
end

-- Get soft cap
local function get_soft_cap(difficulty)
    return CONFIG.SOFT_CAPS[difficulty] or CONFIG.DEFAULT_SOFT_CAP
end

-- Get infamy rank
local function get_infamy_rank()
    return managers.experience and managers.experience:current_rank() or 0
end

-- Calculate soft cap reduction
local function calc_soft_cap_reduction(total_xp, cap)
    if total_xp <= cap then
        return 1.0
    end
    local reduction = CONFIG.SOFT_CAP_REDUCTION - ((total_xp - cap) / total_xp)
    return math.max(reduction, CONFIG.SOFT_CAP_MIN)
end
