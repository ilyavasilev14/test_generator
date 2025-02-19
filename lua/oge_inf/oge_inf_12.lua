-- логика создания заданий пишется на языке программирования Lua
-- больше о создании интерфейсов заданий: https://github.com/ilyavasilev14/test_generator/blob/master/docs/custom_gui.md

-- генерация задания
symbols_list = {"A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
    "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X",
    "Y", "Z", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"
}
are_files_created = false
extension = ""
catalogue = ""
answer = math.random(20, 80)
for _ = 1, math.random(2, 5) do
    extension = extension .. symbols_list[math.random(1, #symbols_list)]
end
for _ = 1, math.random(2, 5) do
    catalogue = catalogue .. symbols_list[math.random(1, #symbols_list)]
end

text = "Сколько файлов с расширением " .. extension .. " объемом более 10 байт содержится в каталоге "
    .. catalogue .. "?\nВ ответе укажите только число."

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
    return "ОГЭ Информатика №12"
end

function get_custom_gui(gui)
    gui:vertical(function (vertical_gui)
        vertical_gui:label(text, 32)
        vertical_gui:horizontal(function (horizontal_gui)
            horizontal_gui:button("Открыть каталог", 32, {500.0, 50}, function ()

                for _ = 1, math.random(2, 4) do
                    local other_catalogue = ""
                    for _ = 1, math.random(2, 5) do
                        other_catalogue = other_catalogue .. symbols_list[math.random(1, #symbols_list)]
                    end
                    new_dir(other_catalogue, false)
                end
                new_dir(catalogue, false)
                for _ = 1, math.random(40, 100) do
                    local file_name = ""
                    for _ = 1, math.random(2, 30) do
                        file_name = file_name .. symbols_list[math.random(1, #symbols_list)]
                    end
                    new_file(catalogue .. "/" .. file_name .. "." .. extension, "", false)
                end
                for _ = 1, math.random(40, 100) do
                    ::regenerate_extension::
                    local other_extension = ""
                    for _ = 1, math.random(2, 10) do
                        other_extension = other_extension .. symbols_list[math.random(1, #symbols_list)]
                    end
                    if extension == other_extension then
                        goto regenerate_extension
                    end
                    local file_name = ""
                    for _ = 1, math.random(2, 30) do
                        file_name = file_name .. symbols_list[math.random(1, #symbols_list)]
                    end
                    new_file(catalogue .. "/" .. file_name .. "." .. other_extension, "", false)
                end
                for _ = 1, answer do
                    local file_name = ""
                    for _ = 1, math.random(2, 30) do
                        file_name = file_name .. symbols_list[math.random(1, #symbols_list)]
                    end

                    local contents = ""
                    for _ = 1, math.random(150, 2000) do
                        contents = contents .. symbols_list[math.random(1, #symbols_list)]
                    end
                    new_file(catalogue .. "/" .. file_name .. "." .. extension, contents, false)
                end
                new_dir("", true)
            end)
        end)
    end)
end

function check_exercise(answer)
    return true
end
