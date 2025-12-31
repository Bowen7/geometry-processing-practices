export async function parseObj(response: Response) {
  const text = await response.text()
  const lines = text.split('\n')
  let vertexCount = 0
  for (const line of lines) {
    if (line.startsWith('f ')) {
      break
    }
    vertexCount++
  }
  const faceCount = lines.length - vertexCount
  const vertices = new Float32Array(vertexCount * 3)
  const faces = new Uint32Array(faceCount * 3)

  let max = 0
  let min = Infinity

  lines.forEach((line, i) => {
    if (line.startsWith('v ')) {
      const [x, y, z] = line.slice(2).split(' ').map(Number)
      vertices[i * 3] = x
      vertices[i * 3 + 1] = y
      vertices[i * 3 + 2] = z
    }
    else {
      const [v1, v2, v3] = line.slice(2).split(' ').map(Number)
      const j = i - vertexCount
      faces[j * 3] = v1 - 1
      faces[j * 3 + 1] = v2 - 1
      faces[j * 3 + 2] = v3 - 1
      max = Math.max(max, v1, v2, v3)
      min = Math.min(min, v1, v2, v3)
    }
  })

  return { vertices, faces }
}
