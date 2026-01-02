-- Main hook for {{name}}
-- This file is loaded early via lib/managers/menumanager

_G.{{name}} = _G.{{name}} or {}
{{name}}.mod_path = ModPath

-- Load localization (must be registered before LocalizationManager initializes)
Hooks:Add("LocalizationManagerPostInit", "{{name}}_localization", function(loc)
    local loc_path = {{name}}.mod_path .. "loc/"
    
    if file.DirectoryExists(loc_path) then
        for _, filename in pairs(file.GetFiles(loc_path)) do
            local str = filename:match('^(.+)%.json$')
            if str and Idstring(str) and Idstring(str):key() == SystemInfo:language():key() then
                loc:load_localization_file(loc_path .. filename)
                break
            end
        end
        loc:load_localization_file(loc_path .. "english.json", false)
    end
end)
