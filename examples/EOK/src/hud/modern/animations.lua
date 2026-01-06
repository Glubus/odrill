-- ============================================================================
-- Modern HUD - Animations
-- Counter and fade animations for modern style
-- ============================================================================

use math::{round, lerp}

-- Count up animation for total XP
local function count_up(text_element, start_val, end_val, duration)
    local t = duration or 0.5
    local elapsed = 0
    while elapsed < t and alive(text_element) do
        local dt = coroutine.yield()
        elapsed = elapsed + dt
        local progress = math.min(elapsed / t, 1)
        local current = lerp(start_val, end_val, progress)
        text_element:set_text(tostring(round(current, 1)))
    end
    if alive(text_element) then
        text_element:set_text(tostring(end_val) .. " »XP")
    end
end

-- Fade up and out
local function fade_up(element, duration)
    local t = duration or 0.8
    while t > 0 and alive(element) do
        local dt = coroutine.yield()
        t = math.clamp(t - dt, 0, 1)
        element:set_alpha(t)
        element:set_y(element:y() - dt * 60)
    end
end

-- Delayed fade
local function delayed_fade(element, delay, duration)
    local t = delay or 0.5
    while t > 0 and alive(element) do
        local dt = coroutine.yield()
        t = math.clamp(t - dt, 0, 0.5)
        element:set_alpha(t * 2)
        element:set_y(element:y() - dt * 50)
    end
end
