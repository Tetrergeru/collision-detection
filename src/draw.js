const DEBUG = false

export function draw(context, objects) {
    let i = 0;
    while (i < objects.length) {
        if (objects[i] < 1.1) {
            context.fillStyle = '#f00';
            context.fillRect(objects[i + 1], objects[i + 2], objects[i + 3], objects[i + 4]);
            i += 5;
        } else if (objects[i] < 2.1) {
            context.fillStyle = '#0f0';
            context.moveTo(objects[i + 1], objects[i + 2])
            context.beginPath()
            context.arc(objects[i + 1], objects[i + 2], objects[i + 3], 0, 2 * Math.PI);
            context.closePath()
            context.fill()

            if (DEBUG) {
                context.strokeRect(
                    objects[i + 1] - objects[i + 3],
                    objects[i + 2] - objects[i + 3],
                    objects[i + 3] * 2,
                    objects[i + 3] * 2,
                );
            }

            i += 4;

        } else {
            context.save()
            context.fillStyle = '#00f';
            const points = objects[i + 1];
            context.beginPath()
            context.moveTo(objects[i + 2], objects[i + 3])
            for (let j = 2; j < points * 2; j += 2) {
                context.lineTo(objects[i + 2 + j], objects[i + 3 + j])
            }
            i += points * 2 + 2;
            context.closePath()
            context.fill()
            context.restore()

            if (DEBUG) {
                context.strokeRect(objects[i], objects[i + 1], objects[i + 2], objects[i + 3]);
            }
            i += 4;
        }
    }
}