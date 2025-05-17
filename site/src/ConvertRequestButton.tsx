import React, { useState } from "react";
import { API_URL } from "./App";

interface IConvertRequestButtonProps {
  name: string;
  from_extension: string;
  post_request_url: string;
  setWaiting: (val: boolean) => void;
}

interface IConvertResponse {
  file_name?: string;
  success?: boolean;
}

export const ConvertRequestButton: React.FC<IConvertRequestButtonProps> = (props) => {

  const [file, setFile] = useState<File>();
  const [responseData, setResponseData] = useState<IConvertResponse>({});

  const handleSubmit: React.FormEventHandler = async (e) => {
    e.preventDefault();

    try {
      setTimeout(() => props.setWaiting(true), 200); // display the wait graphics
      // show the waiting stuff after 100 ms so that we avoid flashing, if the response is quick don"t need to show this waiting stuff

      const formData = new FormData();
      formData.append("file", file ?? "");

      let response = fetch(props.post_request_url, {
        method: "POST",
        body: formData,
      });

      // now start resolving the promise

      response.then(async (resp) => {
        if (resp.ok && resp.status === 200) {
          const data = await resp.json();
          setResponseData(data);
        } else {
          console.error("Request failed:", resp);
        }
        props.setWaiting(false); // done waiting
      });

    } catch (error) {
      console.error("Error sending request:", error);
      props.setWaiting(false); // clean up waiting flag
    }
  };

  const handleFileChange: React.ChangeEventHandler = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!!e.target.files) {
      setFile(e.target.files[0]);
    }
  };

  return (
    <div>
      {responseData.success ?
        <div>
          Received JSON data:
          <pre>{JSON.stringify(responseData, null, 2)}</pre>
          Go here to get your file: <a href={API_URL + (responseData.file_name ?? "")}>Download</a>
        </div>
        :
        <form>
          <label htmlFor={"fileinput_" + props.name}>
            <input
              type="file"
              accept={props.from_extension}
              onChange={handleFileChange}
            />
          </label>
          <div className="_flexBreak"> </div>
          <button className="_button _centerVert" disabled={!file} type="submit" onClick={handleSubmit}>Submit</button>
        </form>}
    </div>
  );
};
