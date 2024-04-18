// import * as React from 'react'
// import { useRef, useState } from 'react'

function App() {

  return (
    <div>
      <GetButton />
      <PostButton />
      <PutButton />
      <DeleteButton />
    </div>
  )
}

function GetButton() {
  return (
    <button id="getbutton" title='GET' onClick={GetAPICall}> GET REQUEST </button>
  )
}

function PostButton() {
  return (
    <button id="postbutton" title='POST' onClick={PostAPICall}> POST REQUEST </button>
  )
}

function PutButton() {
  return (
    <button id="putbutton" title='PUT'> PUT REQUEST </button>
  )
}

function DeleteButton() {
  return (
    <button id="deletebutton" title='DELETE'> DELETE REQUEST </button>
  )
}

function GetAPICall() {
  fetch('http://localhost:5001/', {
    method: "GET"
  })
    .then(response => response.json())
    .then(data => console.log(data));
}

function PostAPICall() {
  fetch('http://localhost:5001/echo', {
    method: "POST",
    body: "this is what I want you to echo."
  })
    .then(response => response.json())
    .then(data => console.log(data));
}
export default App;
