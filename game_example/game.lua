function verdi.start()
    image = graphics.newImage("C:/Users/Julien/Pictures/container.png")
end

function verdi.update()

end

function verdi.draw()
    graphics.beginObject("triangles")
        graphics.normal(1.0, 0.0, 0.0)

        graphics.color(1.0, 0.0, 0.0, 1.0)
        graphics.tex_coord(-0.5, -0.5, 0.0)
        graphics.vertex(-0.5, -0.5, 0.0)

        graphics.tex_coord(0.0, 0.5, 0.0)
        graphics.color(0.0, 1.0, 0.0, 1.0)
        graphics.vertex(0.0, 0.5, 0.0)

        graphics.tex_coord(0.5, -0.5, 0.0)
        graphics.color(0.0, 0.0, 1.0, 1.0)
        graphics.vertex(0.5, -0.5, 0.0)
    graphics.endObject()
end