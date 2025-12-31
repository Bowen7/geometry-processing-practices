export async function parseOff(response: Response) {
  const text = await response.text()
  const lines = text.split('\n')
  const meta = lines[1].split(' ').map(Number)
  const [vertexCount, faceCount] = meta
  const vertices = new Float32Array(vertexCount * 3)
  const faces = new Uint32Array(faceCount * 3)
  for (let i = 0; i < vertexCount; i++) {
    const [x, y, z] = lines[2 + i].split(' ').map(Number)
    vertices[i * 3] = x
    vertices[i * 3 + 1] = y
    vertices[i * 3 + 2] = z
  }
  for (let i = 0; i < faceCount; i++) {
    const [_, v1, v2, v3] = lines[2 + vertexCount + i].split(' ').map(Number)
    faces[i * 3] = v1
    faces[i * 3 + 1] = v2
    faces[i * 3 + 2] = v3
  }
  return { vertices, faces }
}
