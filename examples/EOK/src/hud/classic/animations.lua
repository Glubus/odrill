-- ============================================================================
-- Classic HUD - Animations
-- Complete fade animations for classic style
-- ============================================================================

-- Fade out with movement direction
local function fade_out_move(element, fade_dir, callback)
    local panel = element:parent()
    element:animate(function(o)
        local t = 1
        while t > 0 and alive(element) do
            local dt = coroutine.yield()
            t = math.clamp(t - dt, 0, 1)
            o:move(fade_dir * dt * 20, (1 - math.abs(fade_dir)) * -dt * 20)
            element:set_alpha(t)
        end
        if alive(element) then
            element:hide()
        end
        if callback then callback() end
    end)
end

-- Killstreak slide animation  
local function slide_out(element, callback)
    element:animate(function(o)
        local t = 4
        while t > 0 and alive(element) do
            local dt = coroutine.yield()
            t = math.clamp(t - dt, 0, 0.9)
            o:move(dt * 20, 0)
            element:set_alpha(t / 4)
        end
        if alive(element) then
            element:hide()
        end
        if callback then callback() end
    end)
end

-- Rainbow color anim (looping)
local function animate_rainbow(text_element)
    local t = 5
    while true and alive(text_element) do
        local dt = coroutine.yield()
        t = (t + dt) % 5
        local h = t / 5 * 6
        local i = math.floor(h)
        local f = h - i
        local q = 1 - f
        local sector = i % 6
        
        local r, g, b
        if sector == 0 then r, g, b = 1, f, 0
        elseif sector == 1 then r, g, b = q, 1, 0
        elseif sector == 2 then r, g, b = 0, 1, f
        elseif sector == 3 then r, g, b = 0, q, 1
        elseif sector == 4 then r, g, b = f, 0, 1
        else r, g, b = 1, 0, q end
        
        text_element:set_color(Color(r, g, b))
    end
end
