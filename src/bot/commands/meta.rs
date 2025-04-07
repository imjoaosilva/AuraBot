use serenity::all::{
    CommandInteraction, Context, CreateActionRow, CreateCommand, CreateInputText, CreateInteractionResponse, CreateModal, InputTextStyle
};

pub async fn run(ctx: Context, command: CommandInteraction) {

    let value_input = CreateInputText::new(InputTextStyle::Short, "Valor Entregue", "value")
        .placeholder("Digite a quantidade entegue");

    let who_input = CreateInputText::new(InputTextStyle::Short, "Quem entregou", "who")
        .placeholder("Digite a quem entregadou");

    let value_i_component = CreateActionRow::InputText(value_input);
    let who_i_component = CreateActionRow::InputText(who_input);

    let modal = CreateModal::new("goal", "Entregar Meta").components(vec![
        value_i_component,
        who_i_component,
    ]);

    let response = CreateInteractionResponse::Modal(modal);
    command.create_response(&ctx.http, response).await.unwrap()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("meta")
        .description("Envie infos sobre a meta")
}
