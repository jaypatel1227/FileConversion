import React, { useState } from 'react';
import { ConvertRequestButton } from './ConvertRequestButton'

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
      <div className='_flexBreak'> </div>
      <div className='_cardDescription'> {description} </div>
      <div className='_flexBreak'> </div>
      <div className='_formArea' hidden={waiting}>
        <ConvertRequestButton name={name} fileExtension={fileExtension} postRequestURL={postRequestURL} setWaiting={setWaiting} />
      </div>
      <div hidden={!waiting}>
        Waiting for response...
      </div>
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
