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
            context.fill()
            context.restore()

            if (DEBUG) {
                context.strokeRect(objects[i], objects[i + 1], objects[i + 2], objects[i + 3]);
            }
            i += 4;
        }
    }
}

export function draw_quad_tree(context, tree, x1, y1) {
    console.log(tree.length)
    draw_quad_tree_helper(context, tree, 0.0, 0.0, x1, y1, 0)
}

function draw_quad_tree_helper(context, tree, x0, y0, x1, y1, i) {
    if (tree[i] < 0.1) {
        return i + 1
    } else {
        const xc = x0 + (x1 - x0) / 2
        const yc = y0 + (y1 - y0) / 2

        context.save()
        context.beginPath()
        context.moveTo(xc, y0)
        context.lineTo(xc, y1)

        context.moveTo(x0, yc)
        context.lineTo(x1, yc)

        context.closePath()
        context.stroke()
        context.restore()

        i += 1
        i = draw_quad_tree_helper(context, tree, x0, y0, xc, yc, i)
        i = draw_quad_tree_helper(context, tree, xc, y0, x1, yc, i)
        i = draw_quad_tree_helper(context, tree, x0, yc, xc, y1, i)
        i = draw_quad_tree_helper(context, tree, xc, yc, x1, y1, i)
        return i
    }
}