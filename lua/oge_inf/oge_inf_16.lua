-- логика создания заданий пишется на языке программирования Lua
-- больше о создании интерфейсов заданий: https://github.com/ilyavasilev14/test_generator/blob/master/docs/custom_gui.md

-- генерация задания
local division_num = math.random(2, 5)
local max_number = math.random(25, 500)
local number_count = math.random(10, 200)

local example_numbers = ""
local output = 0
for i = 1, 4 do
    local number = 1
    if i < 3 then
        while number % division_num ~= 0 do
            number = math.random(2, max_number)
        end
        if number > output then
            output = number
        end
    else
        number = math.random(1, max_number)
        while number % division_num == 0 do
            number = math.random(1, max_number)
        end
    end

    example_numbers = example_numbers .. number .. "\n"
end

local text = "Напишите программу на языке программирования Python, которая в последовательности натуральных чисел определяет сумму чисел, кратных "
    .. division_num .. ". Программа получает на вход количество чисел в последовательности, а затем сами числа. В последовательности всегда имеется число, кратное "
    .. division_num ..  ". Количество чисел не превышает 200. Введенные числа не превышают " .. max_number
    .. ". Программа должна вывести одно число - сумму чисел, кратных " .. division_num .. ".\nПример работы программы:"
    .. "\nВходные данные:\n4\n" .. example_numbers .. "Выходные данные:\n" .. output

-- слеующая функция возвращает текст задания
function get_exercise_text()
    return text
end

-- слеующая функция возвращает правильный ответ на задание
function get_exercise_right_answer()
    return nil
end

-- слеующая функция возвращает название генератора заданий
function get_exercise_title()
    return "№16 ОГЭ информатика"
end

function get_custom_gui(gui)
    gui:vertical(function (vertical_gui)
        vertical_gui:label(text, 24)
        vertical_gui:button("Открыть", 30, {410.0, 50}, function ()
            new_dir("", false)
            new_file("solution.py", "", true)
        end)
    end)
end

function check_exercise(_)
    local numbers = tostring(number_count) .. "\n" .. tostring(division_num) .. "\n"
    local answer = division_num
    for _ = 1, number_count - 1 do
        local number = math.random(1, max_number)
        numbers = numbers .. tostring(number) .. "\n"
        if number % division_num == 0 and number > answer then
            answer = number
        end
    end

    local script = 'echo "' .. numbers .. '" | python ' .. get_full_path("solution.py")
    new_file("python_run_script.sh", script, false)
    local output = run_command("sh", get_full_path("python_run_script.sh"))

    local answer_string = tostring(answer)
    for i = 1, #answer_string do
        if string.sub(output, i, i) ~= string.sub(answer_string, i, i) then
            return false
        end
    end
    return true
end
