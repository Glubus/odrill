-- ============================================================================
-- EOK ExperienceManager Hook
-- Tracks kill XP separately for end-screen display
-- ============================================================================

-- ============================================================================
-- KILL XP TRACKING
-- ============================================================================

-- Helper to get the current kill XP total
local function get_kills_xp()
    if Global.experience_manager and Global.experience_manager.kills_xp then
        return Global.experience_manager.kills_xp
    end
    return 0
end

-- Clear kill XP on mission end/start
local function clear_kills_xp()
    if Global.experience_manager then
        Global.experience_manager.kills_xp = nil
    end
end

-- Store kill XP before clearing for display
local stored_kills_xp = 0

-- ============================================================================
-- HOOKS
-- ============================================================================

-- Hook mission_xp_clear to also clear our kill XP
Hooks:PostHook(ExperienceManager, "mission_xp_clear", "EOK_clear_kills_xp", function(self)
    clear_kills_xp()
    stored_kills_xp = 0
end)

-- Hook give_experience to store the kills XP value before it gets cleared
Hooks:PreHook(ExperienceManager, "give_experience", "EOK_store_kills_xp", function(self, xp, force_or_debug)
    stored_kills_xp = get_kills_xp()
end)

-- Hook give_experience to display total kill XP at mission end
Hooks:PostHook(ExperienceManager, "give_experience", "EOK_show_kills_xp_display", function(self, xp, force_or_debug)
    if stored_kills_xp > 0 then
        -- Use DelayedCalls to show after UI is loaded
        DelayedCalls:Add("EOK_delayed_kills_display", 2, function()
            if managers.menu_component and managers.menu_component._game_chat_gui then
                -- Use chat system as fallback to show message
                managers.chat:send_message(ChatManager.GAME, nil, "Kill XP: +" .. tostring(math.round(stored_kills_xp)))
            end
        end)
    end
    
    -- Clear the stored value
    clear_kills_xp()
end)
