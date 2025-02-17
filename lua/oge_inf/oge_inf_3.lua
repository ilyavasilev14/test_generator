local text
local right_answer = ""

-- generating the exercise
local not_number = math.random(0, 150)
local normal_number = not_number + 1

-- 0 is false and 1 is true here
local is_not_number_first = math.random(0, 1)

local exercise_type = math.random(0, 1)

if exercise_type == 0 then
    if is_not_number_first == 1 then
        text = "Напишите целое число X, для которого ложно высказывание:\n(X > ".. normal_number
            .. ") ИЛИ НЕ (X > " .. not_number .. ")"
    else
        text = "Напишите целое число X, для которого ложно высказывание:\nНЕ (X > " .. not_number
            .. ") ИЛИ (X > ".. normal_number .. ")"
    end
    right_answer = tostring(normal_number)
else
    if is_not_number_first == 1 then
        text = "Напишите целое число X, для которого истинно высказывание:\nНЕ (X < ".. not_number
            .. ") И (X < " .. normal_number .. ")"
    else
        text = "Напишите целое число X, для которого истинно высказывание:\n(X < ".. normal_number
            .. ") И НЕ (X < " .. not_number .. ")"
    end
    right_answer = tostring(not_number)
end

function get_exercise_right_answer()
    return right_answer
end

function get_exercise_text()
    return text
end

function get_exercise_title()
    return "№3 ОГЭ информатика"
end

