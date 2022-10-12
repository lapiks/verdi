function verdi.start()
    scene = graphics.newScene("./game_example/assets/tank.gltf")
    print(scene:getNumNodes())
end

function verdi.update()

end

function verdi.draw()
    scene:draw()

    -- for i = 0, scene:getNumNodes() do
    --     local node = scene:getNode(i) 
    --     node:draw()
    -- end

    if input.getKeyDown("z") then
        print("z is down")
    end

    if input.getButtonDown("l") then
        print("left mouse button is down")
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