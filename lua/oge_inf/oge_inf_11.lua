-- логика создания заданий пишется на языке программирования Lua
-- больше о создании интерфейсов заданий: https://github.com/ilyavasilev14/test_generator/blob/master/docs/custom_gui.md

local symbols_list = {"A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
    "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X",
    "Y", "Z", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"
}

-- генерация задания
local answer = ""
local answer_param = ""

for _ = 1, math.random(7, 15) do
    local symbol = math.random(1, #symbols_list)
    answer = answer .. symbols_list[symbol]
end

for _ = 1, math.random(7, 15) do
    local symbol = math.random(1, #symbols_list)
    answer_param = answer_param .. symbols_list[symbol]
end

local lines_count = math.random(100, 500)
local answer_line = math.random(50, lines_count)

local file_data = ""
for i = 1, lines_count do
    if i == answer_line then
        file_data = file_data .. answer_param .. ' = "' .. answer .. '"\n'
    else
        local new_param_name = ""
        for _ = 1, math.random(7, 15) do
            local symbol = math.random(1, #symbols_list)
            new_param_name = new_param_name .. symbols_list[symbol]
        end
        local new_param = ""
        for _ = 1, math.random(7, 15) do
            local symbol = math.random(1, #symbols_list)
            new_param = new_param .. symbols_list[symbol]
        end
        file_data = file_data .. new_param_name .. ' = "' .. new_param .. '"\n'
    end
end

text = 'Откройте файл "params.txt" и найдите, чему равен параметер "' .. answer_param
    .. '"\nВ ответ укажите только цифры и латинские буквы.'

-- слеующая функция возвращает текст задания
function get_exercise_text()
    return text
end

-- слеующая функция возвращает правильный ответ на задание
function get_exercise_right_answer()
    return answer
end

-- слеующая функция возвращает название генератора заданий
function get_exercise_title()
    return "№11 ОГЭ информатика"
end

function get_custom_gui(gui)
    gui:vertical(function (vertical_gui)
        vertical_gui:label(text, 24)
        vertical_gui:button("Открыть", 20, {410, 50}, function ()
            new_dir("", false)
            new_file("params.txt", file_data, true)
        end)
    end)
end

--[[ своя логика проверки задания, опционально
function check_exercise(answer)
end
]]--
