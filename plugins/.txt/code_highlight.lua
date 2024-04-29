Tokens = {
    Default = "White",
    Test = "Yellow",
}

function Get_default()
    return Tokens.Default
end

function Tokenize(chars)
    local tokenarr = {}
    local t = ""

    for i = 1, #chars do
        local c = chars[i]
        if string.match(c, "%a") then
            t = t .. c
        else
            if t == "hello" then
                for _ = 1, #t do
                    table.insert(tokenarr, Tokens.Test)
                end
            else
                for _ = 1, #t do
                    table.insert(tokenarr, Tokens.Default)
                end
            end
            table.insert(tokenarr, Tokens.Default)
            t = ""
        end
    end

    if t ~= "" then
        if t == "hello" then
            for _ = 1, #t do
                table.insert(tokenarr, Tokens.Test)
            end
        else
            for _ = 1, #t do
                table.insert(tokenarr, Tokens.Default)
            end
        end
    end

    return tokenarr
end
