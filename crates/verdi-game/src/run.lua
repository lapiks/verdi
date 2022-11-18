function verdi.run(deltaTime) 
    if verdi.update then verdi.update() end
    if verdi.draw then verdi.draw(deltaTime) end
end