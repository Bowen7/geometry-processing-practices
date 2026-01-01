import type { ThreeEvent } from '@react-three/fiber'
import { Billboard, Center } from '@react-three/drei'
import { useThree } from '@react-three/fiber'
import { useQuery } from '@tanstack/react-query'
import { useGesture } from '@use-gesture/react'
import { BiharmonicHandler } from 'geometry-processing'
import { button, useControls } from 'leva'
import { useEffect, useMemo, useRef, useState } from 'react'
import { Canvas } from '@/components/canvas'
import { Loading } from '@/components/loading'
import { buildGeometry } from '@/lib/geometry'
import { parseOff } from '@/lib/off'

const SCALE = 5

interface ControlHandlerProps {
  v: number
  vertices: Float32Array
  onDrag: (offset: [number, number]) => void
  onDragStart: () => void
}
function ControlHandler({ v, vertices, onDrag, onDragStart }: ControlHandlerProps) {
  const { size, viewport } = useThree()
  const aspect = size.width / viewport.width

  const onClick = (e: ThreeEvent<MouseEvent>) => {
    e.stopPropagation()
  }

  const bind = useGesture({
    onDrag: ({ offset: [x, y] }) => onDrag([x / aspect / SCALE, -y / aspect / SCALE]),
    onDragStart,
  })

  return (
    <mesh
      position={[vertices[v * 3], vertices[v * 3 + 1], vertices[v * 3 + 2]]}
      renderOrder={999}
      onClick={onClick}
      {...bind()}
    >
      <circleGeometry args={[0.02, 16]} />
      <meshPhongMaterial color="#FA5053" depthTest={false} />
    </mesh>
  )
}

function getDistance(a: [number, number, number], b: [number, number, number]) {
  return ((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2 + (a[2] - b[2]) ** 2)
}

function getNearestVertex(vertices: Float32Array, a: number, b: number, c: number, point: [number, number, number]) {
  const pa: [number, number, number] = [vertices[a * 3], vertices[a * 3 + 1], vertices[a * 3 + 2]]
  const pb: [number, number, number] = [vertices[b * 3], vertices[b * 3 + 1], vertices[b * 3 + 2]]
  const pc: [number, number, number] = [vertices[c * 3], vertices[c * 3 + 1], vertices[c * 3 + 2]]
  const distances = [getDistance(pa, point), getDistance(pb, point), getDistance(pc, point)]
  const minDistance = Math.min(...distances)
  return [a, b, c][distances.indexOf(minDistance)]
}

const initialControls = [324, 359, 90, 26, 463]
const initialControlDisplacements = Array.from({ length: initialControls.length * 3 }, () => 0)

function Deformation({ vertices: initialVertices, faces }: { vertices: Float32Array, faces: Uint32Array }) {
  const [vertices, setVertices] = useState(initialVertices)
  const [controls, setControls] = useState<number[]>(initialControls)
  const controlDisplacementsRef = useRef<number[]>(initialControlDisplacements)
  const previousControlDisplacementsRef = useRef<number[]>([])

  const handler = useMemo(() => {
    const b = new Uint32Array(controls)
    const handler = new BiharmonicHandler(initialVertices, faces, b)
    return handler
  }, [initialVertices, faces, controls])

  useEffect(() => {
    return () => {
      handler.free()
    }
  }, [handler])

  useControls(() => ({
    'reset controls': button(() => {
      setControls(initialControls)
      controlDisplacementsRef.current = initialControlDisplacements
    }),
    'clear controls': button(() => {
      setControls([])
      controlDisplacementsRef.current = []
    }),
  }))

  const onClick = (e: ThreeEvent<MouseEvent>) => {
    if (typeof e.face?.a === 'number') {
      const nearestVertex = getNearestVertex(vertices, e.face.a, e.face.b, e.face.c, [e.point.x, e.point.y, e.point.z])
      if (controls.includes(nearestVertex)) {
        return
      }
      setControls([...controls, nearestVertex])
      controlDisplacementsRef.current = [...controlDisplacementsRef.current, 0, 0, 0]
    }
  }

  const onDragStart = () => {
    previousControlDisplacementsRef.current = [...controlDisplacementsRef.current]
  }

  const onDrag = (index: number, offset: [number, number]) => {
    const newBc = [...previousControlDisplacementsRef.current]
    newBc[index * 3] = previousControlDisplacementsRef.current[index * 3] + offset[0]
    newBc[index * 3 + 1] = previousControlDisplacementsRef.current[index * 3 + 1] + offset[1]
    controlDisplacementsRef.current = newBc
    const { displacements } = handler.solve(new Float32Array(controlDisplacementsRef.current))
    const vertices = new Float32Array(initialVertices)
    for (let i = 0; i < vertices.length; i++) {
      vertices[i] += displacements[i]
    }
    setVertices(vertices)
  }

  const geometry = buildGeometry(vertices, faces)

  return (
    <Center scale={SCALE}>
      <mesh geometry={geometry} onClick={onClick}>
        <meshPhongMaterial color="#adebb3" />
      </mesh>
      <Billboard>
        {controls.map((control, index) => (
          <ControlHandler
            key={control}
            v={control}
            vertices={vertices}
            onDragStart={onDragStart}
            onDrag={offset => onDrag(index, offset)}
          />
        ))}
      </Billboard>
    </Center>
  )
}

export function DeformationPractice() {
  const { data } = useQuery({
    queryKey: ['knight'],
    queryFn: () => fetch('data/knight.off').then(parseOff),
  })

  if (!data) {
    return <Loading />
  }

  return (
    <Canvas enableRotate={false}>
      <Deformation vertices={data.vertices} faces={data.faces} />
      <axesHelper />
    </Canvas>
  )
}
