local text
local right_answer = ""
local letter_list = {'А', 'Б', 'В', 'Г'}
local number_list = {'10', '110', '12', '102'}

-- generating the exercise
local numbers_count = math.random(3, 7)
local number_string = ""
for _ = 1, numbers_count do
    local letter_type = math.random(1, #letter_list)
    number_string = number_string .. number_list[letter_type]
    right_answer = right_answer .. letter_list[letter_type]
end
text = "Мальчики играли в шпионов и закодировали сообщение придуманным шифром. "
    .. "В сообщении присутствуют только буквы из приведенного фрагмента кодовой таблицы:"
    .. "\nА = 10\nБ = 110\nВ = 12\nГ = 102\nОпределите, какое сообщение закодировано в строчке:\n"
    .. number_string .. "\nВ ответе запишите последовательность букв без запятых и других знаков препинания."


function get_exercise_right_answer()
    return right_answer
end

function get_exercise_text()
    return text
end

function get_exercise_title()
    return "№2 ОГЭ информатика"
end

