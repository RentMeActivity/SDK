export async function acceptTask(taskId: string) {
  console.log("Human accepted task:", taskId)
}

export async function submitProof(taskId: string, uri: string) {
  console.log("Proof submitted:", uri)
}
