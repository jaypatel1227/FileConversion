import React, { useState } from 'react';
import { API_URL } from './App';

export interface ICardProps {
  name: string;
  description: string;
  fileExtension: string;
  postRequestURL: string;
}

const Card: React.FC<ICardProps> = (props) => {
  const [waiting, setWaiting] = useState(false);
  let { name, description, fileExtension, postRequestURL } = props;

  return (
    <div className='_container'>
      <div className='_cardTitle'> {name} </div>
      <div className='_cardDescription'> {description} </div>
      <div hidden={waiting}>
        <ConvertRequestButton name={name} fileExtension={fileExtension} postRequestURL={postRequestURL} setWaiting={setWaiting} />
      </div>
      <div hidden={!waiting}>
        Waiting for response...
      </div>
    </div>
  );
};

interface IConvertRequestButtonProps {
  name: string;
  fileExtension: string;
  postRequestURL: string;
  setWaiting: (val: boolean) => void;
}

interface IConvertResponse {
  file_name?: string;
  success?: boolean;
}

const ConvertRequestButton: React.FC<IConvertRequestButtonProps> = (props) => {

  const [file, setFile] = useState<File>();
  const [responseData, setResponseData] = useState<IConvertResponse>({});

  const handleSubmit: React.FormEventHandler = async (e) => {
    e.preventDefault();

    try {
      setTimeout(() => props.setWaiting(true), 200); // display the wait graphics
      // show the waiting stuff after 100 ms so that we avoid flashing, if the response is quick don't need to show this waiting stuff

      const formData = new FormData();
      formData.append('file', file ?? '');

      let response = fetch(props.postRequestURL, {
        method: 'POST',
        body: formData,
      });

      // now start resolving the promise

      response.then(async (resp) => {
        if (resp.ok) {
          const data = await resp.json();
          setResponseData(data);
        } else {
          console.error('Request failed:', resp.status);
        }
        props.setWaiting(false); // done waiting
      });

    } catch (error) {
      console.error('Error sending request:', error);
      props.setWaiting(false); // clean up wadone waitingiting flag
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
          Go here to get your file: <a href={API_URL + responseData.file_name!!}>Download</a>
        </div>
        :
        <form>
          <label htmlFor={'fileinput_' + props.name}>
            <input
              type="file"
              accept={props.fileExtension}
              onChange={handleFileChange}
            />
          </label>
          <button type="submit" onClick={handleSubmit}>Submit</button>
        </form>}
    </div>
  );
};

interface ICardRowProps {
  cards: ICardProps[];
  initialIndex: number;
}

const CardRow: React.FC<ICardRowProps> = (props) => {

  return (
    <>
      {props.cards.map((card, index) => (
        <td key={index + props.initialIndex} className='_cardItem'>
          <Card name={card.name} description={card.description} fileExtension={card.fileExtension} postRequestURL={card.postRequestURL} />
        </td>
      ))}
    </>
  );
};

export interface ICardTableProps {
  cards?: ICardProps[];
}

export const CardTable = (props: ICardTableProps) => {

  let NUM_COLS = 3;

  const groupedCards = props.cards?.reduce<ICardProps[][]>((arr, card, index) => {
    if (index % NUM_COLS === 0) arr.push([]);
    arr[arr.length - 1].push(card);
    return arr;
  }, []);

  return (
    <table className='_serviceGrid'>
      <tbody>
        {groupedCards?.map((group, index) => (
          <tr className='_tableRow'>
            <CardRow cards={group} initialIndex={index * NUM_COLS} />
          </tr>
        ))}
      </tbody>
    </table>
  );
};

export default CardTable;
