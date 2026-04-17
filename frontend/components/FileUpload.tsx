'use client'

import { useState, useCallback } from 'react'

interface FileUploadProps {
  onUpload: (files: File[]) => void
  disabled?: boolean
}

export function FileUpload({ onUpload, disabled }: FileUploadProps) {
  const [files, setFiles] = useState<File[]>([])
  const [dragging, setDragging] = useState(false)

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    setDragging(false)
    const droppedFiles = Array.from(e.dataTransfer.files)
    setFiles(prev => [...prev, ...droppedFiles])
  }, [])

  const handleFileInput = (e: React.ChangeEvent<HTMLInputElement>) => {
    const fileList = e.target.files
    if (fileList) {
      const newFiles = Array.from(fileList as ArrayLike<File>)
      setFiles(prev => [...prev, ...newFiles])
    }
  }

  const removeFile = (index: number) => {
    setFiles(prev => prev.filter((_, i) => i !== index))
  }

  const handleSubmit = () => {
    if (files.length > 0) {
      onUpload(files)
    }
  }

  return (
    <div className="w-full">
      <div
        className={`border-2 border-dashed rounded-lg p-8 text-center transition-colors ${
          dragging ? 'border-blue-500 bg-blue-500/10' : 'border-gray-600'
        }`}
        onDragOver={(e) => { e.preventDefault(); setDragging(true) }}
        onDragLeave={() => setDragging(false)}
        onDrop={handleDrop}
      >
        <input
          type="file"
          multiple
          className="hidden"
          id="file-upload"
          onChange={handleFileInput}
          accept=".pdf,.doc,.docx,.xlsx,.xls"
        />
        <label htmlFor="file-upload" className="cursor-pointer">
          <p className="text-gray-400">
            Drag and drop files here, or click to select
          </p>
          <p className="text-gray-500 text-sm mt-2">
            Supported: PDF, DOC, XLSX
          </p>
        </label>
      </div>

      {files.length > 0 && (
        <ul className="mt-4 space-y-2">
          {files.map((file, index) => (
            <li key={index} className="flex items-center justify-between bg-gray-800 p-3 rounded">
              <span className="text-sm truncate flex-1">{file.name}</span>
              <button
                onClick={() => removeFile(index)}
                className="text-red-500 hover:text-red-400 ml-4"
              >
                Remove
              </button>
            </li>
          ))}
        </ul>
      )}

      <button
        onClick={handleSubmit}
        disabled={disabled || files.length === 0}
        className="mt-4 w-full py-3 bg-green-600 hover:bg-green-700 disabled:bg-gray-600 disabled:cursor-not-allowed rounded-lg transition"
      >
        {disabled ? 'Processing...' : 'Submit for Evaluation'}
      </button>
    </div>
  )
}
