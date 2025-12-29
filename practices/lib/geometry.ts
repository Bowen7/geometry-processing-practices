import * as THREE from 'three'

export function buildGeometry(vertices: number[] | Float64Array, faces: number[] | Uint32Array) {
  const geometry = new THREE.BufferGeometry()

  const positions = new Float32Array(vertices)
  const indices = new Uint32Array(faces)

  geometry.setAttribute('position', new THREE.BufferAttribute(positions, 3))
  geometry.setIndex(new THREE.BufferAttribute(indices, 1))
  geometry.computeVertexNormals()
  geometry.center()

  return geometry
}
