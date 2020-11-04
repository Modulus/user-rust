use crate::db::models::{Message, NewMessage, User};
use crate::errors::BackendError;
use crate::utils::lib::date_now;
use diesel::prelude::*;

pub fn send_message(
    from: &User,
    to: &User,
    _header: String,
    _contenty: String,
    conn: &PgConnection,
) {
    let new_message = NewMessage {
        header: _header,
        message: _contenty,
        sender_user_id: from.id,
        receiver_user_id: to.id,
        sent: date_now(),
        modified: None,
    };

    use crate::schema::messages;

    let result = diesel::insert_into(messages::table)
        .values(new_message)
        .execute(conn);

    println!("Result of insert of message: {:?}", result);
}

pub fn list_sent_messages(user: &User, conn: &PgConnection) -> Result<Vec<Message>, BackendError> {
    use crate::schema::messages::dsl::*;

    let result = messages
        .filter(sender_user_id.eq(user.id))
        .limit(25)
        .load::<Message>(conn)?;
    return Ok(result);
}

pub fn list_all_messages(user: &User, conn: &PgConnection) -> Result<Vec<Message>, BackendError> {
    use crate::schema::messages::dsl::*;

    let result = messages
        .filter(sender_user_id.eq(user.id).or(receiver_user_id.eq(user.id)))
        .limit(25)
        .load::<Message>(conn)?;
    return Ok(result);
}

pub fn delete_message(
    owner: &User,
    message_id: i32,
    conn: &PgConnection,
) -> Result<usize, BackendError> {
    use crate::schema::messages::dsl::*;

    println!(
        "Deleting message with id: {:?} for user: {:?}",
        message_id, owner.name
    );
    return Ok(
        diesel::delete(messages.filter(sender_user_id.eq(owner.id).and(id.eq(message_id))))
            .execute(conn)?,
    );
}

// pub fn change_message(
//     from: &User,
//     to: &User,
//     new_header: String,
//     new_content: String,
//     conn: &PgConnection,
// ) {
//     unimplemented!("Not impelemented yet")
// }

#[cfg(test)]
mod tests {
    use crate::db::lib::establish_connection;
    use crate::db::messages::{
        delete_message, list_all_messages, list_sent_messages, send_message,
    };
    use crate::db::models::NewUserJson;
    use crate::db::users::{create_user, delete_user_by_name};

    #[test]
    fn int_crd_messages() {
        let connection = establish_connection();

        let new_user = NewUserJson {
            name: "test-user-messages-1".to_string(),
            comment: "User created from intagration test".to_string(),
            active: true,
            password: "supersecret".to_string(),
        };
        let result1 = create_user(&connection, &new_user);

        assert!(result1.is_ok());

        let new_user = NewUserJson {
            name: "test-user-messages-2".to_string(),
            comment: "User created from intagration test".to_string(),
            active: true,
            password: "supersecret1234".to_string(),
        };
        let result2 = create_user(&connection, &new_user);
        assert!(result2.is_ok());

        let user1 = result1.unwrap();
        let user2 = result2.unwrap();

        send_message(
            &user1,
            &user2,
            "Hi there".to_string(),
            "First message!".to_string(),
            &connection,
        );
        send_message(
            &user2,
            &user1,
            "Re:hi there".to_string(),
            "Kewl!".to_string(),
            &connection,
        );
        send_message(
            &user1,
            &user2,
            "Re:hi there".to_string(),
            "What are you up to these days".to_string(),
            &connection,
        );
        send_message(
            &user2,
            &user1,
            "Re:hi there".to_string(),
            "Learning rust, how about you?".to_string(),
            &connection,
        );
        send_message(
            &user1,
            &user2,
            "Re:hi there".to_string(),
            "Begone deamon!!!! :P".to_string(),
            &connection,
        );

        let messages_user1 = list_sent_messages(&user1, &connection).unwrap();
        assert_eq!(messages_user1.len(), 3);

        let messages_all_user1 = list_all_messages(&user1, &connection).unwrap();
        assert_eq!(messages_all_user1.len(), 5);

        let messages_user2 = list_sent_messages(&user2, &connection).unwrap();
        assert_eq!(messages_user2.len(), 2);

        let messages_all_user2 = list_all_messages(&user2, &connection).unwrap();
        assert_eq!(messages_all_user2.len(), 5);

        for message1 in messages_user1 {
            println!("{:?}", delete_message(&user1, message1.id, &connection));
        }

        for message2 in messages_user2 {
            println!("{:?}", delete_message(&user2, message2.id, &connection));
        }

        let deleted_messages_user1 = list_sent_messages(&user1, &connection);
        // assert_eq!(deleted_messages_user1.len() ,0);
        println!("{:?}", deleted_messages_user1);

        let deleted_messages_user2 = list_sent_messages(&user2, &connection);
        // assert_eq!(deleted_messages_user2.len(), 0);
        println!("{:?}", deleted_messages_user1);

        delete_user_by_name(&connection, &user1.name);
        delete_user_by_name(&connection, &user2.name);
    }
}
