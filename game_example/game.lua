local position = world.newComponent({ x = 0, y = 0, z = 0 })

function verdi.start()
    model = graphics.newModel("./game_example/assets/tank.gltf")
    mesh = graphics.newMesh()
    transform = math.newTransform()
    image = graphics.newImage("./game_example/assets/Palette.png")
    sprite = graphics.newSprite(image)

    entity = world.spawn()
    print(entity:id())

    entity2 = world.spawn()
    print(entity2:id())

    vertices = {
        {0, 0, 0},
        {1, 1, 0},
        {0, 1, 0}
    }

    mesh:setVertices(vertices)
	mesh:setPrimitiveType("triangles")

    print(model:getNumNodes())

    x = 0
    y = 0

    camPitch = 0
    camYaw = 0

    speed = 5
    rotSpeed = 1

    test.func()

    --local source = audio.newSource("./game_example/assets/Mgła - Age of Excuse - 01 Age of Excuse I.mp3")
    --audio.play(source)
end

function verdi.update(deltaTime) 

    local tf = graphics.camera:transform()


    graphics.camera:reset()
    --graphics.camera:setRotation(camYaw, 0, 1, 0)
    graphics.camera:translate(x, 2.5, y)
    graphics.camera:rotate(camYaw, 0, 1, 0)
    graphics.camera:rotate(camPitch, 1, 0, 0)

    if input.getKeyDown("z") then
        y = y + tf:forward() * speed * deltaTime
    end

    if input.getKeyDown("s") then
        y = y - tf:forward() * speed * deltaTime
    end

    if input.getKeyDown("d") then
        x = x + speed * deltaTime
    end

    if input.getKeyDown("q") then
        x = x - speed * deltaTime
    end

    if input.getKeyDown("e") then
        camYaw = camYaw + rotSpeed * deltaTime
    end

    if input.getKeyDown("a") then
        camYaw = camYaw - rotSpeed * deltaTime
    end

    local mouseDelta = {input.getMouseDelta()}
    camYaw = camYaw + mouseDelta[1] * deltaTime * 10
    camPitch = camPitch + mouseDelta[2] * deltaTime * 10

    if input.getButtonDown("l") then

    end

    if input.getKeyDown(" ") then
        print("space is down")
    end
end

function verdi.draw(pass)
    graphics.setClearColor(0.0, 0.0, 0.0, 1.0)

    --transform:translate(5.0, 0.0, 0.0)
    --transform:rotate(1.0, 0.0, 1.0, 0.0)
    --transform:scale(2.0, 2.0, 2.0)

    pass:enableLighting(true)
    pass:enableFog(true)
    pass:setFogStart(10.0)
    pass:setFogEnd(25.0)
    pass:drawModel(model)
    pass:drawMesh(mesh, transform)
    pass:drawSprite(sprite)

    pass:submit(graphics.camera)

    -- for i = 0, model:getNumNodes() do
    --     local node = model:getNode(i) 
    --     node:draw()
    -- end

    --graphics.beginObject("triangles")
        --graphics.bindTexture(image)

        --graphics.normal(1.0, 0.0, 0.0)

        --graphics.color(1.0, 0.0, 0.0, 1.0)
        --graphics.tex_coord(-0.5, -0.5, 0.0)
        --graphics.vertex(-0.5, -0.5, 0.0)

        --graphics.tex_coord(0.0, 0.5, 0.0)
        --graphics.color(0.0, 1.0, 0.0, 1.0)
        --graphics.vertex(0.0, 0.5, 0.0)

        --graphics.tex_coord(0.5, -0.5, 0.0)
        --graphics.color(0.0, 0.0, 1.0, 1.0)
        --graphics.vertex(0.5, -0.5, 0.0)
    --graphics.endObject()
end