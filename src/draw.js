export function draw(context, objects) {
    let i = 0;
    while (i < objects.length) {
        if (objects[i] < 1.1) {
            context.fillStyle = '#f00';
            context.fillRect(objects[i + 1], objects[i + 2], objects[i + 3], objects[i + 4]);
            i += 5;
        } else {
            context.fillStyle = '#0f0';
            context.moveTo(objects[i + 1], objects[i + 2])
            context.beginPath()
            context.arc(objects[i + 1], objects[i + 2], objects[i + 3], 0, 2 * Math.PI);
            context.closePath()
            context.fill()
            i += 4;
        }
    }
}