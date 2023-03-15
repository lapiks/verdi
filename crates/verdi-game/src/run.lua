function verdi.run(deltaTime, pass) 
    if verdi.update then verdi.update(deltaTime) end
    if verdi.draw then verdi.draw(pass) end
end