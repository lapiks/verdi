local position = world.newComponent({ x = 0, y = 0, z = 0 })

function verdi.start()
    model = graphics.newModel("./game_example/assets/tank.gltf")
    mesh = graphics.newMesh()
    transform = math.newTransform()
    image = graphics.newImage("./game_example/assets/Palette.png")
    --sprite = graphics.newSprite(image)

    graphics.camera.transform:setPosition(math.vec3(-2.0, 2.5, -5))

    entity = world.spawn()
    print(entity:id())

    entity2 = world.spawn()
    print(entity2:id())

    vertices = {
        {0, 0, 0},
        {1, 1, 0},
        {0, 1, 0}
    }

    indices = { 0, 1, 2 }

    mesh:setVertices(vertices)
    mesh:setIndices(indices)
	mesh:setPrimitiveType("triangles")

    print(model:getNumNodes())

    camPitch = 0
    camYaw = 0

    test.func()

    --local source = audio.newSource("./game_example/resources/Mgła - Age of Excuse - 01 Age of Excuse I.mp3")
    --audio.play(source)
end

function verdi.update(deltaTime) 
    local camTF = graphics.camera.transform

    local speed = 5
    local rotSpeed = 1

    if input.getKeyDown("z") then
        camTF:translate(camTF:forward() * speed * deltaTime)
    end

    if input.getKeyDown("s") then
        camTF:translate(camTF:backward() * speed * deltaTime)
    end

    if input.getKeyDown("d") then
        camTF:translate(camTF:right() * speed * deltaTime)
    end

    if input.getKeyDown("q") then
        camTF:translate(camTF:left() * speed * deltaTime)
    end

    if input.getKeyDown("e") then
        camYaw = camYaw + rotSpeed * deltaTime
    end

    if input.getKeyDown("a") then
        camYaw = camYaw - rotSpeed * deltaTime
    end

    -- camTF:reset()
    -- camTF:setRotation(camYaw, 0, 1, 0)
    -- camTF:translate(camPos)
    -- camTF:rotate(camYaw, 0, 1, 0)

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
    --graphics.setClearColor(0.0, 0.0, 0.0, 1.0)

    pass:enableLighting(true)
    pass:enableFog(true)
    pass:setFogStart(10.0)
    pass:setFogEnd(25.0)
    pass:drawModel(model)
    pass:drawMesh(mesh, transform)
    --pass:drawSprite(sprite)

    pass:submit(graphics.camera)

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