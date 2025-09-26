import React from 'react'
import ReactFlow, { Background, MiniMap, Controls } from 'reactflow'

const initialNodes = [
  {
    id: '1',
    data: { label: 'ETL Start' },
    position: { x: 250, y: 5 },
  },
]

export default function App() {
  return (
    <div style={{ height: '100vh' }}>
      <ReactFlow nodes={initialNodes} fitView>
        <Background />
        <MiniMap />
        <Controls />
      </ReactFlow>
    </div>
  )
}
