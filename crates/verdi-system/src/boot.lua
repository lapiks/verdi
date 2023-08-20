function verdi.boot()
    -- default camera creation
    local cam_transform = math.newTransform()
    graphics.camera = graphics.newCamera(cam_transform)

    if verdi.start then verdi.start() end
end