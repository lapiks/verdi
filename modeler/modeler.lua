function verdi.start()
    mesh = graphics.newMesh()

    vertices = {
        {0, 0, 0},
        {1, 1, 0},
        {0, 1, 0}
    }

    mesh:setVertices(vertices)
	mesh:setPrimitiveType("triangles")

    transform = math.newTransform()

    x = 5
    y = 10

    camPitch = 0
    camYaw = 0

    speed = 5
    rotSpeed = 1
end

function verdi.update(deltaTime) 
    graphics.camera:reset()
    graphics.camera:setPosition(x, -2.5, y)
    graphics.camera:rotate(camYaw, 0, 1, 0)
    graphics.camera:rotate(camPitch, 1, 0, 0)

    if input.getKeyDown("z") then
        y = y - speed * deltaTime
    end

    if input.getKeyDown("s") then
        y = y + speed * deltaTime
    end

    if input.getKeyDown("q") then
        x = x + speed * deltaTime
    end

    if input.getKeyDown("d") then
        x = x - speed * deltaTime
    end

    if input.getKeyDown("a") then
        camYaw = camYaw + rotSpeed * deltaTime
    end

    if input.getKeyDown("e") then
        camYaw = camYaw - rotSpeed * deltaTime
    end

end

function verdi.draw(pass)
    graphics.setClearColor(0.0, 0.0, 0.0, 1.0)

    pass:enableLighting(false)
    pass:drawMesh(mesh, transform)

    pass:submit(graphics.camera)
end