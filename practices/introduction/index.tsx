import type { LineSegments2 } from 'three/addons/lines/LineSegments2.js'
import { Center, Line } from '@react-three/drei'
import { useQuery } from '@tanstack/react-query'
import { wasm_edges, wasm_euler_characteristic } from 'geometry-processing'
import { useControls } from 'leva'
import { useEffect, useRef } from 'react'
import { Canvas } from '@/components/canvas'
import { Loading } from '@/components/loading'
import { buildGeometry } from '@/lib/geometry'
import { parseOff } from '@/lib/off'

interface BunnyProps {
  vertices: Float32Array
  faces: Uint32Array
}
function Bunny({ vertices, faces }: BunnyProps) {
  const geometry = buildGeometry(vertices, faces)
  return (
    <Center>
      <mesh geometry={geometry} scale={20}>
        <meshPhongMaterial />
      </mesh>
    </Center>
  )
}

function getLines(edges: Uint32Array, vertices: Float32Array) {
  const lines: [number, number, number][] = []
  for (let i = 0; i < edges.length / 2; i++) {
    const v1 = edges[i * 2]
    const v2 = edges[i * 2 + 1]
    lines.push([vertices[v1 * 3], vertices[v1 * 3 + 1], vertices[v1 * 3 + 2]])
    lines.push([vertices[v2 * 3], vertices[v2 * 3 + 1], vertices[v2 * 3 + 2]])
  }
  return lines
}

type EdgesProps = BunnyProps
function Edges({ vertices, faces }: EdgesProps) {
  const { edges } = wasm_edges(faces, vertices.length / 3)
  const lines = getLines(edges, vertices)
  const lineRef = useRef<LineSegments2>(null)

  useEffect(() => {
    if (lineRef.current) {
      lineRef.current.geometry.center()
    }
  }, [])

  return (
    <group scale={20}>
      <Line points={lines} segments ref={lineRef} />
    </group>
  )
}

export function IntroductionPractice() {
  const { data } = useQuery({
    queryKey: ['bunny'],
    queryFn: () => fetch('data/bunny.off').then(parseOff),
  })

  const chi = data ? Number(wasm_euler_characteristic(data.vertices, data.faces).chi) : 0
  const [{ edges }, set] = useControls(() => ({ edges: false, Chi: { value: chi, disabled: true, step: 1 } }))

  useEffect(() => {
    set({ Chi: chi })
  }, [chi, set])

  if (!data) {
    return <Loading />
  }

  return (
    <Canvas>
      {edges
        ? <Edges vertices={data.vertices} faces={data.faces} />
        : <Bunny vertices={data.vertices} faces={data.faces} />}
      <axesHelper />
    </Canvas>
  )
}
