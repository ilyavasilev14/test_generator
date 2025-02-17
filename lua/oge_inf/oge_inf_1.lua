local symbols_list = {"A", "B", "C", "D", "E", "F", "G", "H", "I", "J",
    "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X",
    "Y", "Z", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0"
}

local text
local right_answer

-- generating the exercise
local excersise_type = math.random(0, 2)
local symbol_size = ""
local reduced_size = 0
local strings_list = ""

local strings_count = math.random(4, 6);
local reduced_size_symbols_count = math.random(2, 8);
local reduced_size_id = math.random(1, strings_count);
local strings = {};
for i = 1, strings_count do
    local symbols_count
    if i ~= reduced_size_id then
        symbols_count = math.random(3, 6);
        while symbols_count == reduced_size_symbols_count do
            symbols_count = math.random(3, 6);
        end
    else
        symbols_count = reduced_size_symbols_count
    end

    for _ = 1, symbols_count do
        local new_symbol_idx = math.random(1, #symbols_list)
        local new_symbol = symbols_list[new_symbol_idx]
        if strings[i] == nil then
            strings[i] = ""
        end
        strings[i] = strings[i] .. new_symbol
    end
    strings_list = strings_list .. strings[i]
    if i < strings_count then
        strings_list = strings_list .. ", "
    end
end
right_answer = strings[reduced_size_id]

if excersise_type == 0 then
    reduced_size = #(strings[reduced_size_id]) + 2
    symbol_size = "КОИ-8 кодируется 8 битами"
elseif excersise_type == 1 then
    reduced_size = (#(strings[reduced_size_id]) + 2) * 2
    symbol_size = "UTF-16 кодируется 16 битами"
elseif excersise_type == 2 then
    reduced_size = (#(strings[reduced_size_id]) + 2) * 4
    symbol_size = "UTF-32 кодируется 32 битами"
end

text = "Каждый символ в кодировке " .. symbol_size .. ". Миша написал текст (в нем нет лишних пробелов):\n«" ..
    strings_list .. " - наборы символов».\nУченик вычеркнул из списка один из наборов символов. Заодно он вычеркнул " ..
    "ставшие лишними запятые и пробелы - два пробела не должны идти подряд.\nПри этом размер нового предложения в данной кодировке оказался на " ..
    reduced_size .. " байтов меньше, чем размер исходного предложения.\nНапишите в ответе вычеркнутый набор символов."



function get_exercise_right_answer()
    return right_answer
end

function get_exercise_text()
    return text
end

function get_exercise_title()
    return "№1 ОГЭ информатика"
end

