-- Chat hook for {{name}}
-- Sends Hello World message when ChatManager is initialized

Hooks:PostHook(ChatManager, "init", "{{name}}_chat_init", function(self)
    DelayedCalls:Add("{{name}}_hello", 3, function()
        if managers.chat then
            local message = managers.localization:text("{{name}}_hello_world")
            managers.chat:_receive_message(1, "[{{name}}]", message, Color.white)
        end
    end)
end)
