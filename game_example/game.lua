local position = world.newComponent({ x = 0, y = 0, z = 0 })

function verdi.start()
    scene = graphics.newModel("./game_example/assets/tank.gltf")
    mesh = graphics.newMesh()
    transform = math.newTransform()

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

    print(scene:getNumNodes())

    x = 5
    y = 10

    camPitch = 0
    camYaw = 0

    speed = 5
    rotSpeed = 1

    test.func()

    local audioClip = audio.newClip("./game_example/assets/Mgła - Age of Excuse - 01 Age of Excuse I.mp3")
    audio.play(audioClip)
end

function verdi.update() 

end

function verdi.draw(deltaTime)
    graphics.setClearColor(0.0, 0.0, 0.0, 1.0)

	graphics.enableLighting(true)

    --graphics.line(0.0, 0.0, 1.0, 1.0)

    graphics.enableFog(true)
    graphics.setFogStart(10.0)
    graphics.setFogEnd(25.0)

    graphics.rotate(camPitch, 1, 0, 0)
    graphics.rotate(camYaw, 0, 1, 0)
    graphics.translate(x, -2.5, y)

    transform:translate(5.0, 0.0, 0.0)
    transform:rotate(1.0, 0.0, 1.0, 0.0)
    transform:scale(2.0, 2.0, 2.0)

    scene:draw()
    mesh:draw()

    -- for i = 0, scene:getNumNodes() do
    --     local node = scene:getNode(i) 
    --     node:draw()
    -- end
    
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

    -- local mouseDelta = {input.getMouseDelta()}
    -- camYaw = camYaw + mouseDelta[1] * deltaTime
    -- camPitch = camPitch + mouseDelta[2] * deltaTime

    if input.getButtonDown("l") then

    end

    if input.getKeyDown(" ") then
        print("space is down")
    end

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