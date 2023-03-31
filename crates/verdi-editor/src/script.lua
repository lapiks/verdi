function verdi.start()
    model = graphics.newModel("./game_example/assets/tank.gltf")
    transform = math.newTransform()
    image = graphics.newImage("./game_example/assets/Palette.png")
    sprite = graphics.newSprite(image)

    x = 5
    y = 10

    camPitch = 0
    camYaw = 0

    speed = 5
    rotSpeed = 1
end

function verdi.update(deltaTime) 
    graphics.camera:setPosition(x, -2.5, y)
    graphics.camera:setRotation(camYaw, 0, 1, 0)
    graphics.camera:setRotation(camPitch, 1, 0, 0)

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

    if input.getButtonDown("l") then

    end

    if input.getKeyDown(" ") then
        print("space is down")
    end
end

function verdi.draw(pass)
    graphics.setClearColor(0.0, 0.0, 0.0, 1.0)

    pass:drawModel(model)
    pass:drawSprite(sprite)

    pass:submit(graphics.camera)
end