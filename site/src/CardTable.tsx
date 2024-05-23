import React, { useState } from 'react';

export interface ICardProps {
  name: string;
  description: string;
  fileExtension: string;
  postRequestURL: string;
}

const Card: React.FC<ICardProps> = (props) => {
  let { name, description, fileExtension, postRequestURL } = props;

  return (
    <div className='_container'>
      <div className='_cardTitle'> {name} </div>
      <div className='_cardDescription'> {description} </div>
      <ConvertRequestButton name={name} fileExtension={fileExtension} postRequestURL={postRequestURL} />
    </div>
  );
};

interface IConvertRequestButtonProps {
  name: string;
  fileExtension: string;
  postRequestURL: string;
}

const ConvertRequestButton: React.FC<IConvertRequestButtonProps> = (props) => {

  const [file, setFile] = useState<File>();
  const [responseData, setResponseData] = useState(null);

  const handleSubmit: React.FormEventHandler = async (e) => {
    e.preventDefault();

    try {
      const formData = new FormData();
      formData.append('file', file ?? '');

      const response = await fetch(props.postRequestURL, {
        method: 'POST',
        body: formData,
      });

      if (response.ok) {
        const data = await response.json();
        setResponseData(data);
      } else {
        console.error('Request failed:', response.status);
      }
    } catch (error) {
      console.error('Error sending request:', error);
    }
  };

  const handleFileChange: React.ChangeEventHandler = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (!!e.target.files) {
      setFile(e.target.files[0]);
    }
  };

  return (
    <div>
      <form>
        <label htmlFor={'fileinput_' + props.name}>
          <input
            type="file"
            accept={props.fileExtension}
            onChange={handleFileChange}
          />
        </label>
        <button type="submit" onClick={handleSubmit}>Submit</button>
      </form>
      {responseData && (
        <div>
          Received JSON data:
          <pre>{JSON.stringify(responseData, null, 2)}</pre>
        </div>
      )}
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
