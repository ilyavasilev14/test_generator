local text

-- generating the exercise
local base_number = math.random(1, 150)
local plus_number = math.random(1, 25)
local rounded_half_plus = math.floor(plus_number / 2 + 1 + 0.5)
local multiply_number = math.random(2, rounded_half_plus + 1)
local plus_count = math.random(1, 4)
local operations = {0, 0, 0, 0, 0}

for i = 1, plus_count do
    operations[i] = 1
end

operations[plus_count] = 2
for i = plus_count + 1, 5 do
    operations[i] = 1
end

local value = base_number
local program = ""
for _, i in pairs(operations) do
    if i == 1 then
        value = value + plus_number
        program = program .. "1"
    elseif i == 2 then
        value = value * multiply_number
        program = program .. "2"
    end
end

text = "У исполнителя Альфа две команды, которым присвоены номера:\n"
    .. "1. прибавь " .. plus_number .. ";\n2. умножь на b\n(b - неизвестное натуральное число;"
    .. " b >= 2).\n\nВыполняя первую из них, Альфа увеличивает число на экране "
    .. "на " .. plus_number .. ", а выполняя вторую, умножает это число на b. Программа для исполнителя "
    .. "Альфа - это последовательность номеров команд. Известно, что программа " .. program .. " переводит число "
    .. base_number .. " в число " .. value .. ". Определите значение b."

function get_exercise_right_answer()
    return tostring(multiply_number)
end

function get_exercise_text()
    return text
end

function get_exercise_title()
    return "№5 ОГЭ информатика"
end

